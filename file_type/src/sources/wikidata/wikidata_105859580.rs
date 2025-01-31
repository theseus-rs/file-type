use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_105859580: FileFormat = FileFormat {
    id: 105_859_580,
    puid: "wikidata/105859580",
    name: "ParaView VTK Unstructured grid",
    extensions: &["vtu"],
    media_types: &["text/xml"],
    internal_signatures: &[],
    related_formats: &[],
};
