use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28445579: FileFormat = FileFormat {
    id: 28_445_579,
    puid: "wikidata/28445579",
    name: "AcqKnowledge File",
    extensions: &["acq"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
