using Microsoft.EntityFrameworkCore;

var builder = WebApplication.CreateBuilder(args);

builder.Services.AddDbContext<CommentDatabase>(opt =>
{
    opt.UseSqlite("Data Source=commenter_dotnet.db");
});

// Add services to the container.
builder.Services.AddRazorPages();

var app = builder.Build();

app.UseStaticFiles();

app.UseRouting();

app.UseAuthorization();

app.MapRazorPages();

app.MapPost("/submit", async (Comment comment, CommentDatabase db) =>
{
    db.Comments.Add(comment);
    await db.SaveChangesAsync();

    return Results.Ok();
});

app.Run();
