use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_114133839: FileFormat = FileFormat {
    id: 114_133_839,
    puid: "wikidata/114133839",
    name: "MacroModels file format",
    extensions: &["mcm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
