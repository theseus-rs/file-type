use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_4836515: FileFormat = FileFormat {
    id: 4_836_515,
    source_type: SourceType::Wikidata,
    name: "BSAVE",
    extensions: &["bsv", "cgx", "pic", "scn"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
