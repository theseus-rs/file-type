use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_105859861: FileFormat = FileFormat {
    id: 105_859_861,
    source_type: SourceType::Wikidata,
    name: "VBScript Encoded script (with rem)",
    extensions: &["vbe"],
    media_types: &["text/vbscript.encode"],
    signatures: &[],
    related_formats: &[],
};
