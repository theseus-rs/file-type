use crate::FileType;
use crate::format::{FileFormat, SourceType};

pub(crate) const WIKIDATA_83795336: FileType = FileType {
    file_format: &FileFormat {
        id: 83_795_336,
        source_type: SourceType::Wikidata,
        name: "ZFO (Sent Message) File",
        extensions: &["zfo"],
        media_types: &["application/vnd.software602.filler.form-xml-zip"],
        signatures: &[],
        related_formats: &[],
    },
};
