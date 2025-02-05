use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_130358117: FileFormat = FileFormat {
    id: 130_358_117,
    source_type: SourceType::Wikidata,
    name: "Mosel source code file",
    extensions: &["mos"],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
