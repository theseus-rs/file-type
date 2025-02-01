use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28757839: FileFormat = FileFormat {
    id: 28_757_839,
    puid: "wikidata/28757839",
    name: "Genecyst Backup RAM",
    extensions: &["gsv"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
