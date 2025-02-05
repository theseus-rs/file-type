use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27967086: FileFormat = FileFormat {
    id: 27_967_086,
    source_type: SourceType::Wikidata,
    name: "Soundfactory",
    extensions: &["psf"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
