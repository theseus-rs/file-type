use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_32110: FileType = FileType {
    file_format: &FileFormat {
        id: 32_110,
        source_type: SourceType::Wikidata,
        name: "XSLT",
        extensions: &["xsl", "xslt"],
        media_types: &["application/xslt+xml", "text/xsl"],
        signatures: &[],
        related_formats: &[],
    },
};
