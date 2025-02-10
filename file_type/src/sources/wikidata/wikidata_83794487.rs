use crate::format::{FileFormat, SourceType};
use crate::FileType;

pub(crate) const WIKIDATA_83794487: FileType = FileType {
    file_format: &FileFormat {
        id: 83_794_487,
        source_type: SourceType::Wikidata,
        name: "ZFO (Form) File",
        extensions: &["zfo"],
        media_types: &["application/vnd.software602.filler.form-xml-zip"],
        signatures: &[],
        related_formats: &[],
    },
};
