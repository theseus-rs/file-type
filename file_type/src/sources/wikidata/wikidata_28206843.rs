use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_28206843: FileFormat = FileFormat {
    id: 28_206_843,
    puid: "wikidata/28206843",
    name: "Palm Database ImageViewer",
    extensions: &["pdb"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
