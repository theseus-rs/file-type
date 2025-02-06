use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27979400: FileFormat = FileFormat {
    id: 27_979_400,
    source_type: SourceType::Wikidata,
    name: "JPX",
    extensions: &["jpf", "jpx"],
    media_types: &["image/jpx"],
    signatures: &[],
    related_formats: &[],
};
