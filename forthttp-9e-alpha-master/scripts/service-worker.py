import requests

code = """
<script>
// Check if the service worker is supported
if ('serviceWorker' in navigator) {
  // Register a service worker
  navigator.serviceWorker.register('/service-worker.js').then(function(registration) {
    // Set the scope to the service worker script
    registration.scope = '/service-worker.js';
    // Fetch the script asynchronously
    registration.update().then(function() {
      // Successfully registered and updated
      navigator.serviceWorker.ready.then(function() {
        console.log('Service worker registered:', registration);
      });
    });
  }).catch(function(error) {
    console.error('Error registering service worker:', error);
  });
}
</script>
<script>
// Get the container element
var container = document.getElementById('main-container');
container.addEventListener('service-worker-container', function(event) {
  // Retrieve the service worker script
  navigator.serviceWorker.register('/service-worker.js').then(function(registration) {
    // Set the service worker
    container.log('Service worker registered:', registration);
  });
});
</script>
"""

response = requests.post('http://localhost:8080/forth', data=code)
print(response.text)
