use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105861878: FileFormat = FileFormat {
    id: 105_861_878,
    puid: "wikidata/105861878",
    name: "NeXtMidas Macro (with rem)",
    extensions: &["mm"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
