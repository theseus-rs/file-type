use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_61964244: FileFormat = FileFormat {
    id: 61_964_244,
    source_type: SourceType::Wikidata,
    name: "pulse EKKO header file",
    extensions: &["hd"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
