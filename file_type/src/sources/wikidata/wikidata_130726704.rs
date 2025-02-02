use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_130726704: FileFormat = FileFormat {
    id: 130_726_704,
    source_type: SourceType::Wikidata,
    name: "SARL source code file",
    extensions: &["sarl"],
    media_types: &["text/x-sarl"],
    internal_signatures: &[],
    related_formats: &[],
};
