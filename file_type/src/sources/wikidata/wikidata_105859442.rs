use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859442: FileFormat = FileFormat {
    id: 105_859_442,
    puid: "wikidata/105859442",
    name: "Q Light Controller+ Workspace",
    extensions: &["qxw"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
