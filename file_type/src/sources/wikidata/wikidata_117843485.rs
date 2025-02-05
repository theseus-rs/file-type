use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_117843485: FileFormat = FileFormat {
    id: 117_843_485,
    source_type: SourceType::Wikidata,
    name: "Faxable PCX",
    extensions: &["fcx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
