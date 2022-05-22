using Microsoft.EntityFrameworkCore;

public class CommentDatabase : DbContext
{
    public CommentDatabase(DbContextOptions<CommentDatabase> options)
        : base(options) {}
    
    public DbSet<Comment> Comments => Set<Comment>();
}