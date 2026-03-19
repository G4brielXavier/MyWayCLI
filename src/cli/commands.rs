use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum Commands {

    /// Show informations about MYWAY
    Hey,


    /// Add a new project to WAY
    Add,


    /// Show all existent projects on WAY
    Way {
        #[arg(short, long)]
        oneline: bool,

        #[arg(short, long)]
        complex: bool,

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>
    },


    /// Remove an existent project from WAY
    Giveup {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>

    },


    /// Define a project as "F"(Finish)
    Finish {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

    },


    /// Edit an existent project
    Edit {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

    },
    

    /// Edit a project's status to custom status (as 'stable', 'deprecated', ...)
    Status {
        
        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,
        
    },


    /// List and Add versions to an existent project
    Version {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        list: bool,

        #[arg(short, long)]
        add: bool,

    },



    /// List all stacks that is used in all project
    Stacks,


    /// An danger area to 'kill' (archive) an existent project
    Graveyard {

        #[arg(short, long)]
        uuid: Option<String>,

        #[arg(short, long)]
        name: Option<String>,

        #[arg(short, long)]
        list: bool,

        #[arg(short, long)]
        kill: bool,

        #[arg(short, long)]
        exject: bool
    
    },


    /// Filter all project as 'Latest', 'Newest', 'Finished' or 'Specific Stack'
    Filter,

}