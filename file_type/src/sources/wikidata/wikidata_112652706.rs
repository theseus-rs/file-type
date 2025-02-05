use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_112652706: FileFormat = FileFormat {
    id: 112_652_706,
    source_type: SourceType::Wikidata,
    name: "Astound Video Project file",
    extensions: &["prj"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
