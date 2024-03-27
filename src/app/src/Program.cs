using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using Iconify.Extensions;
using app;
using app.Services.Extension;
using app.Services.FileWatcher;
using app.Services.HotReload;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });
builder.Services.AddIconify();
builder.Services.AddSingleton<ExtensionRegistry>();
builder.Services.AddSingleton<ExtensionManager>();
builder.Services.AddScoped<FileWatcher>();
builder.Services.AddScoped<HotReload>();

await builder.Build().RunAsync();