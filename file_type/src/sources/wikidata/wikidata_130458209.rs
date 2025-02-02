use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130458209: FileFormat = FileFormat {
    id: 130_458_209,
    source_type: SourceType::Wikidata,
    name: "Pan source code file",
    extensions: &["pan"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
