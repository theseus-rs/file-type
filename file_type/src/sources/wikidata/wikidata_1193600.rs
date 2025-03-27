use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_1193600: FileType = FileType {
    file_format: &FileFormat {
        id: 1_193_600,
        source_type: SourceType::Wikidata,
        name: "Markdown",
        extensions: &["markdown", "md", "mdown", "mdtext", "mdtxt", "mkd"],
        media_types: &["text/markdown"],
        signatures: &[],
        related_formats: &[],
    },
};
