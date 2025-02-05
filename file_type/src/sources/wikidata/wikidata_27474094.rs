use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27474094: FileFormat = FileFormat {
    id: 27_474_094,
    source_type: SourceType::Wikidata,
    name: "BIL/BIP/BSQ Statistics File",
    extensions: &["stx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
