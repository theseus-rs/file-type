use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_62561174: FileFormat = FileFormat {
    id: 62_561_174,
    puid: "wikidata/62561174",
    name: "Pagemaker Document",
    extensions: &["p65", "pmd", "pmt"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
