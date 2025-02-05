use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_111729468: FileFormat = FileFormat {
    id: 111_729_468,
    source_type: SourceType::Wikidata,
    name: "RegMon File",
    extensions: &["rgd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
