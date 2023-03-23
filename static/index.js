(() => {

	const main = () => {
  console.group('URL Info');
    console.log('Protocol', window.location.protocol);
    console.log('Host', window.origin);
    console.log('Path', window.location.pathname);
    console.groupCollapsed('Meta Info');
      console.log('Date Fetched', new Date().getTime());
      console.log('OS', navigator['platform']);
      console.log('Browser', navigator['appCodeName']);
      console.log('Language', navigator['language']);
    console.groupEnd();
  console.groupEnd();
	};


	window.onload = main;
})();
