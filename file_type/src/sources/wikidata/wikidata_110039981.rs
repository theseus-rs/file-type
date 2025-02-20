use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_110039981: FileType = FileType {
    file_format: &FileFormat {
        id: 110_039_981,
        source_type: SourceType::Wikidata,
        name: "Phantom CINE Compressed Video File",
        extensions: &["cci"],
        media_types: &[],
        signatures: &[],
        related_formats: &[],
    },
};
