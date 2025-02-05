use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_47538629: FileFormat = FileFormat {
    id: 47_538_629,
    source_type: SourceType::Wikidata,
    name: "AutoCAD Colour-Dependant Plot Style Table",
    extensions: &["ctb"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
