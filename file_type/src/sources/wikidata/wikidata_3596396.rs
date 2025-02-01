use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_3596396: FileFormat = FileFormat {
    id: 3_596_396,
    puid: "wikidata/3596396",
    name: "MDX",
    extensions: &["mdx"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
