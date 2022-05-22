using Microsoft.AspNetCore.Mvc;
using Microsoft.AspNetCore.Mvc.RazorPages;
using Microsoft.EntityFrameworkCore;

namespace Commenter.Dotnet.Pages;

public class IndexModel : PageModel
{
    private readonly ILogger<IndexModel> _logger;
    private readonly CommentDatabase _context;

    public List<Comment> Comments { get; set; } = new();

    public IndexModel(ILogger<IndexModel> logger, CommentDatabase context)
    {
        _logger = logger;
        _context = context;
    }

    public async Task OnGetAsync()
    {
        Comments = await _context.Comments.ToListAsync();
    }
}
