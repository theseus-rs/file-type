use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27473828: FileFormat = FileFormat {
    id: 27_473_828,
    source_type: SourceType::Wikidata,
    name: "BIL/BIP/BSQ Color File",
    extensions: &["clr"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
