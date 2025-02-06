use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116941808: FileFormat = FileFormat {
    id: 116_941_808,
    source_type: SourceType::Wikidata,
    name: "Ulead Template Extension",
    extensions: &["tpx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
