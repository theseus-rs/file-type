use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_116619333: FileFormat = FileFormat {
    id: 116_619_333,
    source_type: SourceType::Wikidata,
    name: "Amiga SVX",
    extensions: &["svx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
