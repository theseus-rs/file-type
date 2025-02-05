use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28975631: FileFormat = FileFormat {
    id: 28_975_631,
    source_type: SourceType::Wikidata,
    name: "Moray User Defined Object",
    extensions: &["udo"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
