<p>Barebone fork of the C++ screencapture library included in https://github.com/smasherprog/screen_capture_lite</p>
<p>I removed everything that is not C++ related to keep the repository small</p>

<p>Master is where development happens and should NOT be considered stable. Use tags for stable releases.</p> 
<p>Window/Linux/Mac <img src="https://smasherprog.visualstudio.com/Smasherprog_projects/_apis/build/status/smasherprog.screen_capture_lite?branchName=master"/><p>
<p>Cross-platform screen and window capturing library<p>
<h2>No External Dependencies except:</h2>
<p>linux: sudo apt-get install libxtst-dev libxinerama-dev libx11-dev libxfixes-dev</p>
<h4>Platforms supported:</h4>

<ul>
<li>Windows 7 and Up</li>
<li>MacOS</li>
<li>Linux</li>
</ul>

<p>The image format is raw BGRA 32 bits per pixel. Alpha is unused for onNewFrame and onFrameChanged except for onMouseChanged where it IS USED! <p>
<p>The data exists like this if you were to march through with a for loop [A,R,G,B], [A,R,G,B], [A,R,G,B]. For a read on why this is check out the post here <a href="https://stackoverflow.com/questions/8104461/pixelformat-format32bppargb-seems-to-have-wrong-byte-order">post here</a><p>

<h2>Examples</h2>
c++
<p>https://github.com/smasherprog/screen_capture_lite/blob/master/Example_CPP/Screen_Capture_Example.cpp</p>

```c++
//Setup Screen Capture for all monitors
auto framgrabber =  SL::Screen_Capture::CreateCaptureConfiguration([]() {
//add your own custom filtering here if you want to capture only some monitors
    return SL::Screen_Capture::SCL_GetMonitors();
  })->onFrameChanged([&](const SL::Screen_Capture::Image& img, const SL::Screen_Capture::Monitor& monitor) {
  
  })->onNewFrame([&](const SL::Screen_Capture::Image& img, const SL::Screen_Capture::Monitor& monitor) {
  
  })->onMouseChanged([&](const SL::Screen_Capture::Image* img, const SL::Screen_Capture::MousePoint &mousepoint) {
  
  })->start_capturing();

framgrabber->SCL_SetFrameChangeInterval(std::chrono::milliseconds(100));//100 ms
framgrabber->SCL_SetMouseChangeInterval(std::chrono::milliseconds(100));//100 ms


//Setup Screen Capture for windows that have the title "cmake" in it
auto windowframgrabber =  SL::Screen_Capture::CreateCaptureConfiguration([]() {
  auto windows = SL::Screen_Capture::SCL_GetWindows();
  std::string srchterm = "cmake";
  // convert to lower case for easier comparisons
  std::transform(srchterm.begin(), srchterm.end(), srchterm.begin(), [](char c) { return std::tolower(c, std::locale());});
  decltype(windows) filtereditems;
  for(auto& a : windows) {
    std::string name = a.Name;
    std::transform(name.begin(), name.end(), name.begin(), [](char c) {return std::tolower(c, std::locale()); });
    if(name.find(srchterm) != std::string::npos) {
      filtereditems.push_back(a);
    }
  }
  return filtereditems;
  })->onFrameChanged([&](const SL::Screen_Capture::Image& img, const SL::Screen_Capture::Window& window) {
  
  })->onNewFrame([&](const SL::Screen_Capture::Image& img, const SL::Screen_Capture::Window& window) {
  
  })->onMouseChanged([&](const SL::Screen_Capture::Image* img, const SL::Screen_Capture::MousePoint &mousepoint) {
  
  })->start_capturing();

windowframgrabber->SCL_SetFrameChangeInterval(std::chrono::milliseconds(100));//100 ms
windowframgrabber->SCL_SetMouseChangeInterval(std::chrono::milliseconds(100));//100 ms

```

<h3>Library Usage</h3>
<p>Only define what are are interested in. Do not define a callback for onMouseChanged if you dont want that information. If you do, the library will assume that you want mouse information and monitor that --so DONT!</p>
<p>Again, DONT DEFINE CALLBACKS FOR EVENTS YOU DONT CARE ABOUT. If you do, the library will do extra work assuming you want the information.</p>
<p>The library owns all image data so if you want to use it for your own purpose after the callback has completed you MUST copy the data out!</p>
<p>Each monitor or window will run in its own thread so there is no blocking or internal synchronization. If you are capturing three monitors, a thread is capturing each monitor.</p>
<h4>ICaptureConfiguration</h4>
<p>Calls to ICaptureConfiguration cannot be changed after start_capturing is called. You must destroy it and recreate it!</p>
<ul>
    <li>
    ICaptureConfiguration::onNewFrame: This will call back when a new frame is ready on the interval specified in SCL_SetFrameChangeInterval
    </li>
    <li>
    ICaptureConfiguration::onFrameChanged: This will call back when differences are detected between the last frame and the current one. This is usefull when you want to stream data that you are only sending what has changed, not everything!
    </li>
    <li>
    ICaptureConfiguration::onMouseChanged: This will call back when the mouse has changed location or the mouse image has changed up to a maximum rate specified in SCL_SetMouseChangeInterval
    </li>
</ul>
<h4>IScreenCaptureManager</h4>
<p>Calls to IScreenCaptureManager can be changed at any time from any thread as all calls are thread safe!</p>
<ul>
    <li>
    IScreenCaptureManager::SCL_SetFrameChangeInterval: This will set the maximum rate that the library will attempt to capture frame events.
    </li>
    <li>
 IScreenCaptureManager::SCL_SetMouseChangeInterval: This will set the maximum rate that the library will attempt to capture mouse events.
    </li>
    <li>
    IScreenCaptureManager::pause: all threads will stop capturing.
    </li>
    <li>
    IScreenCaptureManager::SCL_IsPaused: obvious!
    </li>
    <li>
    IScreenCaptureManager::SCL_Resume: all threads will SCL_Resume capturing.
    </li>
</ul>
