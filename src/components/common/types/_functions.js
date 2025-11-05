
export const spinnerSrcBase64 = "data:image/png;base64,R0lGODdhEAAJAIAAAMLCwsLCwiwAAAAAEAAJAAACCoSPqcvtD6OclBUAOw==";
export const spinnerSrcBase64White = "data:image/png;base64,R0lGODlhAQABAAAAACH5BAEKAAEALAAAAAABAAEAAAICTAEAOw==";
export const validate = function (choice, cookie) {
  if (!cookie || !cookie.choices) return null;
  // console.log('cookie',cookie);
  const choices = Object.keys(choice)
  const chosen = Object.keys(cookie.choices)

  if (chosen.length !== choices.length) {
    return false
  }

  return chosen.every(c => choices.includes(c))
}
export function getBrowser() {
  // Note that navigator is defined in the browser, but not on the server.
  const agent = navigator.userAgent;

  // The order matters here.
  if (agent.includes("Firefox")) return "Firefox";
  if (agent.includes("Trident")) return "Internet Explorer";
  if (agent.includes("Edge")) return " Edge";
  if (agent.includes("Chrome")) return "Chrome";
  if (agent.includes("Safari")) return "Safari";
  return "unknown";
}
export const getAbsoluteDomainUrl = () => {
  /*
   * Function to get Current Domain Url
   * Samples:
   *      "https://domain.sharepoint.com"
   */
  if (window?.location?.protocol && window?.location?.host) {
    return window.location.protocol + "//" + window.location.host;
  }
  return null;
};

export const getCurrentAbsoluteSiteUrl = () => {
  /*
   * Function to get Current Site Url
   * Samples:
   *      "https://domain.sharepoint.com/sites/intranet/subsite/Pages/Home.aspx"
   */
  if (window?.location?.protocol &&
    window?.location?.host &&
    window?.location?.pathname
  ) {
    return (
      window.location.protocol + "//" +
      window.location.host +
      window.location.pathname
    );
  }
  return null;
};
export const getWebServerRelativeUrl = () => {
  /*
   * Function to get Current Site Url
   * Samples:
   *      "/sites/intranet"
   */
  if (window?.location?.pathname) {
    return window.location.pathname.replace(/\/$/, "");
  }
  return null;
};
export const getWebServerSearchUrl = () => {
  /*
   * Function to get Current Site Url
   * Samples:
   *      "/sites/intranet"
   */
  if (window?.location?.search) {
    return window.location.search.replace(/\/$/, "");
  }
  return null;
};
export const getUrlServer = (service = "") => {
  let url =
    "https://bucksmanage.de/react-service.php?service=" +
    service +
    "&access_token=VMoxiRv9pyrb6MvwMMYAfNbRiU63QcCnREf4R8FVhAazP3RLDt4AGt79PFsJKhhM";
  if(window?.location?.host === 'localhost:3000' || 
		 window?.location?.host === 'localhost:3001' || 
		window?.location?.host === 'localhost:5173' || 
		window?.location?.host === '10.237.74.55:5173' 
  ) {
   // let  url = "https://bucksmanage.test/data/"+service+".json";
    url = "http://localhost:8080/?"+service+".json";
  }
  if (window?.location?.host === 'bucksmanage'+'.test') {
    // url = "https://bucksmanage.test/data/"+service+".json";
    url = "https://bucksmanage.test/react-service.php?service=" +
      service +
      "&access_token=VMoxiRv9pyrb6MvwMMYAfNbRiU63QcCnREf4R8FVhAazP3RLDt4AGt79PFsJKhhM";
  }
  return url;
};
export const veloVerlauf = 'data:image/png;base64,';
 export const bucksmanageLogo = 'data:image/png;base64,';

