use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_129326955: FileFormat = FileFormat {
    id: 129_326_955,
    source_type: SourceType::Wikidata,
    name: "GDScript source code file",
    extensions: &["gd"],
    media_types: &["application/x-gdscript", "text/x-gdscript"],
    signatures: &[],
    related_formats: &[],
};
