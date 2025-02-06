use crate::format::{ByteSequence, FileFormat, PositionType, Regex, Signature, SourceType, Token};

pub(crate) const WIKIDATA_28205846: FileFormat = FileFormat {
    id: 28_205_846,
    source_type: SourceType::Wikidata,
    name: "ColoRIX",
    extensions: &[
        "rix", "sca", "scb", "scc", "sce", "scg", "sci", "sck", "scl", "scn", "sco", "scp", "scq",
        "scr", "sct", "scu", "scv", "scw", "scx", "scy", "scz",
    ],
    media_types: &[],
    signatures: &[],
    related_formats: &[],
};
