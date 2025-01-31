use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_27966906: FileFormat = FileFormat {
    id: 27_966_906,
    puid: "wikidata/27966906",
    name: "MDX",
    extensions: &["mdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
