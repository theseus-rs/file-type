use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_127785591: FileFormat = FileFormat {
    id: 127_785_591,
    puid: "wikidata/127785591",
    name: "MetaPost PostScript file",
    extensions: &["mps"],
    media_types: &[],
    internal_signatures: &[],
    related_formats: &[],
};
