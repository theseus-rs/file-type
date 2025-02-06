use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_62561174: FileFormat = FileFormat {
    id: 62_561_174,
    source_type: SourceType::Wikidata,
    name: "Pagemaker Document",
    extensions: &["p65", "pmd", "pmt"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
