use crate::format::{
    ByteSequence, FileFormat, InternalSignature, PositionType, Regex, SourceType, Token,
};

pub(crate) const WIKIDATA_27978800: FileFormat = FileFormat {
    id: 27_978_800,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Smooshed",
    extensions: &["sps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
