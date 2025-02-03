use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_111355515: FileFormat = FileFormat {
    id: 111_355_515,
    source_type: SourceType::Wikidata,
    name: "Talking Technology Incorporated file",
    extensions: &["vox"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
