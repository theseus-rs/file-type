use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27473828: FileFormat = FileFormat {
    id: 27_473_828,
    source_type: SourceType::Wikidata,
    name: "BIL/BIP/BSQ Color File",
    extensions: &["clr"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
