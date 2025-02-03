use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_112652706: FileFormat = FileFormat {
    id: 112_652_706,
    source_type: SourceType::Wikidata,
    name: "Astound Video Project file",
    extensions: &["prj"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
