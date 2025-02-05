use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_27966906: FileFormat = FileFormat {
    id: 27_966_906,
    source_type: SourceType::Wikidata,
    name: "MDX",
    extensions: &["mdx"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
