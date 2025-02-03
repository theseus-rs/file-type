use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27967086: FileFormat = FileFormat {
    id: 27_967_086,
    source_type: SourceType::Wikidata,
    name: "Soundfactory",
    extensions: &["psf"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
