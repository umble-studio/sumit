using Microsoft.AspNetCore.Components.Web;
using Microsoft.AspNetCore.Components.WebAssembly.Hosting;
using app;
using app.Services.Extension;
using Iconify.Extensions;
using app.Services.FileWatcher;
using app.Services;

var builder = WebAssemblyHostBuilder.CreateDefault(args);
builder.RootComponents.Add<App>("#app");
builder.RootComponents.Add<HeadOutlet>("head::after");

builder.Services.AddScoped(sp => new HttpClient { BaseAddress = new Uri(builder.HostEnvironment.BaseAddress) });
builder.Services.AddIconify();
builder.Services.AddSingleton<ExtensionRegistry>();
builder.Services.AddSingleton<ExtensionManager>();
builder.Services.AddScoped<FileWatcher>();
builder.Services.AddScoped<Hotreload>();

await builder.Build().RunAsync();