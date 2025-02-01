use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105858206: FileFormat = FileFormat {
    id: 105_858_206,
    puid: "wikidata/105858206",
    name: "IAR Embedded Workbench Workspace",
    extensions: &["eww"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
