use crate::format::{ByteSequence, FileFormat, InternalSignature, PositionType, Regex, Token};

pub(crate) const WIKIDATA_50259355: FileFormat = FileFormat {
    id: 50_259_355,
    puid: "wikidata/50259355",
    name: "Microsoft Visio Macro-Enabled Stencil, version 2013",
    extensions: &["vssm"],
    media_types: &["application/vnd.ms-visio.stencil.macroEnabled.main+xml"],
    internal_signatures: &[],
    related_formats: &[],
};
