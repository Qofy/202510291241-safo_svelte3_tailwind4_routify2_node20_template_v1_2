import * as stores from './store'
import { get } from 'svelte/store'
import { beforeUrlChange } from './helpers'
import { urlToRoute } from './utils/urlToRoute'
import { currentLocation } from './utils'

export function init(routes, callback) {
    /** @type { ClientNode | false } */
    let lastRoute = false
    // console.log('init', routes, callback)
    function updatePage(proxyToUrl, shallow) {
        // console.log('proxyToUrl', proxyToUrl)
        // console.log('currentLocation().fullpath', currentLocation().fullpath)
        const url = 'https://bucksmanage.test' + (proxyToUrl || currentLocation().fullpath)
        // console.log('url', url)
        const route = urlToRoute(url)

        if (route.redirectTo) {
            // console.log('redirectTo', redirectTo)
            history.replaceStateNative({}, null, route.redirectTo)
            delete route.redirectTo
        }

        const currentRoute = shallow && urlToRoute('https://bucksmanage.test' + currentLocation().fullpath, routes)
        const contextRoute = currentRoute || route
        const nodes = [...contextRoute.layouts, route]
        if (lastRoute) delete lastRoute.last //todo is a page component the right place for the previous route?
        route.last = lastRoute
        lastRoute = route

        //set the route in the store
        if (!proxyToUrl) {
            // console.log('!proxyToUrl urlRoute.set(route', route)
            stores.urlRoute.set(route)
        }
        // console.log('proxyToUrl urlRoute.set(route', route)
        stores.route.set(route)

        //preload components in parallel
        route.api.preload().then(() => {
            // console.log('!preload', 'isChangingPage.set(true)', nodes)
            //run callback in Router.svelte
            stores.isChangingPage.set(true)
            callback(nodes)
        })
    }

    const destroy = createEventListeners(updatePage)

    return { updatePage, destroy }
}

/**
 * svelte:window events doesn't work on refresh
 * @param {Function} updatePage
 */
function createEventListeners(updatePage) {
    // history.*state
    ;['pushState', 'replaceState'].forEach(eventName => {
        // console.log('eventName', eventName)
        if (!history[eventName + 'Native'])
            history[eventName + 'Native'] = history[eventName]
        history[eventName] = async function (state = {}, title, url) {
            // console.log('history[eventName]', eventName, title, url)
            // do nothing if we're navigating to the current page
            const currentUrl = location.pathname + location.search + location.hash
            if (url === currentUrl) return false

            const { id, path, params } = get(stores.route)
            state = { id, path, params, ...state }
            const event = new Event(eventName.toLowerCase())
            Object.assign(event, { state, title, url })

            const route = await runHooksBeforeUrlChange(event, url)
            if (route) {
                history[eventName + 'Native'].apply(this, [state, title, url])
                return dispatchEvent(event)
            }
        }
    })

    let _ignoreNextPop = false

    const listeners = {
        click: handleClick,
        pushstate: () => updatePage(),
        replacestate: () => updatePage(),
        popstate: async event => {
            if (_ignoreNextPop) {
                // console.log('updatePage _ignoreNextPop', event, currentLocation().fullpath)
                _ignoreNextPop = false
            } else {
                if (await runHooksBeforeUrlChange(event, 'https://bucksmanage.test' + currentLocation().fullpath)) {
                    // console.log('updatePage !_ignoreNextPop', event, currentLocation().fullpath)
                    updatePage()
                } else {
                    _ignoreNextPop = true
                    // console.log('updatePage !_ignoreNextPop else', event, currentLocation().fullpath)
                    event.preventDefault()
                    history.go(1)
                }
            }
        },
    }

    Object.entries(listeners).forEach(args => addEventListener(...args))

    const unregister = () => {
        Object.entries(listeners).forEach(args => removeEventListener(...args))
    }

    return unregister
}

function handleClick(event) {
    const el = event.target.closest('a')
    const href = el && el.href
    // console.log('href', href)

    if (
        event.ctrlKey ||
        event.metaKey ||
        event.altKey ||
        event.shiftKey ||
        event.button ||
        event.defaultPrevented
    ) {
        // console.log('handleClick return event.* ', href)
        return
    }
    if (!href || el.target || el.host !== location.host) {
        // console.log('handleClick return !href host==host href', href)
        // console.log('handleClick return !href host==host el.target', el?.target)
        // console.log('handleClick return !href host==host el.host', el?.host)
        // console.log('handleClick return !href host==host location.host', location?.host)
        return
    }
    const url = new URL(href)
    const relativeUrl = url.host + url.pathname + url.search + url.hash
    // console.log('relativeUrl', relativeUrl)
    event.preventDefault()
    history.pushState({}, '', relativeUrl)
}

async function runHooksBeforeUrlChange(event, url) {
    const route = urlToRoute(url).api
    // console.log('runHooksBeforeUrlChange route ', route, event, url)
    for (const hook of beforeUrlChange._hooks.filter(Boolean)) {
        // return false if the hook returns false
        // console.log('runHooksBeforeUrlChange filter ', event, url)
        const result = await hook(event, route, { url })
        if (!result) return false
    }
    return true
}

