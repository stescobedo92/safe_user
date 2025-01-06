IF OBJECT_ID('[dbo].[users]', 'U') IS NOT NULL
DROP TABLE [dbo].[users];
GO

CREATE TABLE [dbo].[users](
    [id] UNIQUEIDENTIFIER NOT NULL DEFAULT NEWID(),
    [UserId] NVARCHAR(50) NOT NULL,
    [Name] NVARCHAR(50) NOT NULL,
    [LastName] NVARCHAR(50) NOT NULL,
    [Email] NVARCHAR(100) NOT NULL,
    [Age] INT NOT NULL,
    [Phone] NVARCHAR(20) NOT NULL,
    [Address] NVARCHAR(100) NULL,
    [BirthDate] DATE NOT NULL,
    [PlaceBirth] NVARCHAR(100) NULL,

    CONSTRAINT [PK_users] PRIMARY KEY CLUSTERED ([id] ASC)
    );
GO