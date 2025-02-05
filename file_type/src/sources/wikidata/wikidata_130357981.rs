use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130357981: FileFormat = FileFormat {
    id: 130_357_981,
    source_type: SourceType::Wikidata,
    name: "MoonScript source code file",
    extensions: &["moon"],
    media_types: &["application/x-moonscript", "text/x-moonscript"],
    signatures: &[],
    related_formats: &[],
};
