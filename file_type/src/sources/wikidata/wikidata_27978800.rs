use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27978800: FileFormat = FileFormat {
    id: 27_978_800,
    source_type: SourceType::Wikidata,
    name: "Spectrum 512 Smooshed",
    extensions: &["sps"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
