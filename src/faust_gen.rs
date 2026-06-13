/* ------------------------------------------------------------
name: "gest_synth"
Code generated with Faust 2.70.3 (https://faust.grame.fr)
Compilation options: -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

use super::{F32, F64, FaustDsp, Meta, ParamIndex, UI};

pub struct mydspSIG0 {
	iVec1: [i32;2],
	iRec2: [i32;2],
}

impl mydspSIG0 {
	
	fn get_num_inputsmydspSIG0(&self) -> i32 {
		return 0;
	}
	fn get_num_outputsmydspSIG0(&self) -> i32 {
		return 1;
	}
	
	fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
		for l2 in 0..2 {
			self.iVec1[l2 as usize] = 0;
		}
		for l3 in 0..2 {
			self.iRec2[l3 as usize] = 0;
		}
	}
	
	fn fillmydspSIG0(&mut self, count: i32, table: &mut[F32]) {
		for i1 in 0..count {
			self.iVec1[0] = 1;
			self.iRec2[0] = (i32::wrapping_add(self.iVec1[1], self.iRec2[1])) % 65536;
			table[i1 as usize] = F32::sin(9.58738e-05 * (self.iRec2[0]) as F32);
			self.iVec1[1] = self.iVec1[0];
			self.iRec2[1] = self.iRec2[0];
		}
	}

}


pub fn newmydspSIG0() -> mydspSIG0 { 
	mydspSIG0 {
		iVec1: [0;2],
		iRec2: [0;2],
	}
}
fn mydsp_faustpower2_f(value: F32) -> F32 {
    value * value
}
static mut ftbl0mydspSIG0: [F32;65536] = [0.0;65536];

#[repr(C)]
pub struct mydsp {
	iVec0: [i32;2],
	fHslider0: F32,
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	iRec1: [i32;2],
	fHslider1: F32,
	fHslider2: F32,
	fConst2: F32,
	fRec3: [F32;2],
	fHslider3: F32,
	fHslider4: F32,
	fRec5: [F32;2],
	fVec2: [F32;2],
	IOTA0: i32,
	fVec3: [F32;4096],
	fConst3: F32,
	fConst4: F32,
	fRec4: [F32;2],
	fConst5: F32,
	fRec6: [F32;2],
	fRec8: [F32;2],
	fRec10: [F32;2],
	fVec4: [F32;2],
	fVec5: [F32;4096],
	fRec9: [F32;2],
	fConst6: F32,
	fRec11: [F32;2],
	fConst7: F32,
	fRec13: [F32;2],
	fHslider5: F32,
	fRec15: [F32;2],
	fVec6: [F32;2],
	fVec7: [F32;4096],
	fRec14: [F32;2],
	fRec16: [F32;2],
	fRec18: [F32;2],
	fVec8: [F32;2],
	fRec0: [F32;3],
	fHslider6: F32,
	fConst8: F32,
	fRec20: [F32;3],
	fRec19: [F32;3],
	fRec22: [F32;2],
	fRec21: [F32;3],
	fHslider7: F32,
	fRec32: [F32;2],
	fVec9: [F32;8192],
	iConst9: i32,
	fRec31: [F32;2],
	fRec34: [F32;2],
	fVec10: [F32;8192],
	iConst10: i32,
	fRec33: [F32;2],
	fRec36: [F32;2],
	fVec11: [F32;8192],
	iConst11: i32,
	fRec35: [F32;2],
	fRec38: [F32;2],
	fVec12: [F32;8192],
	iConst12: i32,
	fRec37: [F32;2],
	fRec40: [F32;2],
	fVec13: [F32;8192],
	iConst13: i32,
	fRec39: [F32;2],
	fRec42: [F32;2],
	fVec14: [F32;8192],
	iConst14: i32,
	fRec41: [F32;2],
	fRec44: [F32;2],
	fVec15: [F32;8192],
	iConst15: i32,
	fRec43: [F32;2],
	fRec46: [F32;2],
	fVec16: [F32;8192],
	iConst16: i32,
	fRec45: [F32;2],
	fVec17: [F32;2048],
	iConst17: i32,
	fRec29: [F32;2],
	fVec18: [F32;2048],
	iConst18: i32,
	fRec27: [F32;2],
	fVec19: [F32;2048],
	iConst19: i32,
	fRec25: [F32;2],
	fVec20: [F32;1024],
	iConst20: i32,
	fRec23: [F32;2],
	fHslider8: F32,
	fHslider9: F32,
}

impl FaustDsp for mydsp {
	type T = F32;
		
	fn new() -> mydsp { 
		mydsp {
			iVec0: [0;2],
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			iRec1: [0;2],
			fHslider1: 0.0,
			fHslider2: 0.0,
			fConst2: 0.0,
			fRec3: [0.0;2],
			fHslider3: 0.0,
			fHslider4: 0.0,
			fRec5: [0.0;2],
			fVec2: [0.0;2],
			IOTA0: 0,
			fVec3: [0.0;4096],
			fConst3: 0.0,
			fConst4: 0.0,
			fRec4: [0.0;2],
			fConst5: 0.0,
			fRec6: [0.0;2],
			fRec8: [0.0;2],
			fRec10: [0.0;2],
			fVec4: [0.0;2],
			fVec5: [0.0;4096],
			fRec9: [0.0;2],
			fConst6: 0.0,
			fRec11: [0.0;2],
			fConst7: 0.0,
			fRec13: [0.0;2],
			fHslider5: 0.0,
			fRec15: [0.0;2],
			fVec6: [0.0;2],
			fVec7: [0.0;4096],
			fRec14: [0.0;2],
			fRec16: [0.0;2],
			fRec18: [0.0;2],
			fVec8: [0.0;2],
			fRec0: [0.0;3],
			fHslider6: 0.0,
			fConst8: 0.0,
			fRec20: [0.0;3],
			fRec19: [0.0;3],
			fRec22: [0.0;2],
			fRec21: [0.0;3],
			fHslider7: 0.0,
			fRec32: [0.0;2],
			fVec9: [0.0;8192],
			iConst9: 0,
			fRec31: [0.0;2],
			fRec34: [0.0;2],
			fVec10: [0.0;8192],
			iConst10: 0,
			fRec33: [0.0;2],
			fRec36: [0.0;2],
			fVec11: [0.0;8192],
			iConst11: 0,
			fRec35: [0.0;2],
			fRec38: [0.0;2],
			fVec12: [0.0;8192],
			iConst12: 0,
			fRec37: [0.0;2],
			fRec40: [0.0;2],
			fVec13: [0.0;8192],
			iConst13: 0,
			fRec39: [0.0;2],
			fRec42: [0.0;2],
			fVec14: [0.0;8192],
			iConst14: 0,
			fRec41: [0.0;2],
			fRec44: [0.0;2],
			fVec15: [0.0;8192],
			iConst15: 0,
			fRec43: [0.0;2],
			fRec46: [0.0;2],
			fVec16: [0.0;8192],
			iConst16: 0,
			fRec45: [0.0;2],
			fVec17: [0.0;2048],
			iConst17: 0,
			fRec29: [0.0;2],
			fVec18: [0.0;2048],
			iConst18: 0,
			fRec27: [0.0;2],
			fVec19: [0.0;2048],
			iConst19: 0,
			fRec25: [0.0;2],
			fVec20: [0.0;1024],
			iConst20: 0,
			fRec23: [0.0;2],
			fHslider8: 0.0,
			fHslider9: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.12.0");
		m.declare("compile_options", r"-lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"gest_synth.dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/pole:author", r"Julius O. Smith III");
		m.declare("filters.lib/pole:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/pole:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.7.0");
		m.declare("name", r"gest_synth");
		m.declare("noises.lib/name", r"Faust Noise Generator Library");
		m.declare("noises.lib/version", r"1.4.0");
		m.declare("oscillators.lib/lf_sawpos:author", r"Bart Brouns, revised by Stéphane Letz");
		m.declare("oscillators.lib/lf_sawpos:licence", r"STK-4.3");
		m.declare("oscillators.lib/name", r"Faust Oscillator Library");
		m.declare("oscillators.lib/saw2ptr:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/saw2ptr:license", r"STK-4.3");
		m.declare("oscillators.lib/sawN:author", r"Julius O. Smith III");
		m.declare("oscillators.lib/sawN:license", r"STK-4.3");
		m.declare("oscillators.lib/version", r"1.5.0");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/mono_freeverb:author", r"Romain Michon");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.1");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 0;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
		let mut sig0: mydspSIG0 = newmydspSIG0();
		sig0.instance_initmydspSIG0(sample_rate);
		sig0.fillmydspSIG0(65536, unsafe { &mut ftbl0mydspSIG0 });
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 2e+03;
		self.fHslider1 = 0.0;
		self.fHslider2 = 3.0;
		self.fHslider3 = 0.05;
		self.fHslider4 = 4.4e+02;
		self.fHslider5 = 1.001;
		self.fHslider6 = 0.0;
		self.fHslider7 = 0.15;
		self.fHslider8 = 0.3;
		self.fHslider9 = 0.5;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.iVec0[l0 as usize] = 0;
		}
		for l1 in 0..2 {
			self.iRec1[l1 as usize] = 0;
		}
		for l4 in 0..2 {
			self.fRec3[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec5[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fVec2[l6 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l7 in 0..4096 {
			self.fVec3[l7 as usize] = 0.0;
		}
		for l8 in 0..2 {
			self.fRec4[l8 as usize] = 0.0;
		}
		for l9 in 0..2 {
			self.fRec6[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec8[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec10[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fVec4[l12 as usize] = 0.0;
		}
		for l13 in 0..4096 {
			self.fVec5[l13 as usize] = 0.0;
		}
		for l14 in 0..2 {
			self.fRec9[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec11[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec13[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec15[l17 as usize] = 0.0;
		}
		for l18 in 0..2 {
			self.fVec6[l18 as usize] = 0.0;
		}
		for l19 in 0..4096 {
			self.fVec7[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec14[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec16[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec18[l22 as usize] = 0.0;
		}
		for l23 in 0..2 {
			self.fVec8[l23 as usize] = 0.0;
		}
		for l24 in 0..3 {
			self.fRec0[l24 as usize] = 0.0;
		}
		for l25 in 0..3 {
			self.fRec20[l25 as usize] = 0.0;
		}
		for l26 in 0..3 {
			self.fRec19[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec22[l27 as usize] = 0.0;
		}
		for l28 in 0..3 {
			self.fRec21[l28 as usize] = 0.0;
		}
		for l29 in 0..2 {
			self.fRec32[l29 as usize] = 0.0;
		}
		for l30 in 0..8192 {
			self.fVec9[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec31[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec34[l32 as usize] = 0.0;
		}
		for l33 in 0..8192 {
			self.fVec10[l33 as usize] = 0.0;
		}
		for l34 in 0..2 {
			self.fRec33[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec36[l35 as usize] = 0.0;
		}
		for l36 in 0..8192 {
			self.fVec11[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec35[l37 as usize] = 0.0;
		}
		for l38 in 0..2 {
			self.fRec38[l38 as usize] = 0.0;
		}
		for l39 in 0..8192 {
			self.fVec12[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec37[l40 as usize] = 0.0;
		}
		for l41 in 0..2 {
			self.fRec40[l41 as usize] = 0.0;
		}
		for l42 in 0..8192 {
			self.fVec13[l42 as usize] = 0.0;
		}
		for l43 in 0..2 {
			self.fRec39[l43 as usize] = 0.0;
		}
		for l44 in 0..2 {
			self.fRec42[l44 as usize] = 0.0;
		}
		for l45 in 0..8192 {
			self.fVec14[l45 as usize] = 0.0;
		}
		for l46 in 0..2 {
			self.fRec41[l46 as usize] = 0.0;
		}
		for l47 in 0..2 {
			self.fRec44[l47 as usize] = 0.0;
		}
		for l48 in 0..8192 {
			self.fVec15[l48 as usize] = 0.0;
		}
		for l49 in 0..2 {
			self.fRec43[l49 as usize] = 0.0;
		}
		for l50 in 0..2 {
			self.fRec46[l50 as usize] = 0.0;
		}
		for l51 in 0..8192 {
			self.fVec16[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.fRec45[l52 as usize] = 0.0;
		}
		for l53 in 0..2048 {
			self.fVec17[l53 as usize] = 0.0;
		}
		for l54 in 0..2 {
			self.fRec29[l54 as usize] = 0.0;
		}
		for l55 in 0..2048 {
			self.fVec18[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec27[l56 as usize] = 0.0;
		}
		for l57 in 0..2048 {
			self.fVec19[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec25[l58 as usize] = 0.0;
		}
		for l59 in 0..1024 {
			self.fVec20[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec23[l60 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 3.1415927 / self.fConst0;
		self.fConst2 = 1.0 / self.fConst0;
		self.fConst3 = 0.5 * self.fConst0;
		self.fConst4 = 0.25 * self.fConst0;
		self.fConst5 = 4.0 / self.fConst0;
		self.fConst6 = 2.0 / self.fConst0;
		self.fConst7 = 0.5 / self.fConst0;
		self.fConst8 = 4.3982296 / self.fConst0;
		self.iConst9 = i32::wrapping_add((0.036666665 * self.fConst0) as i32, 23);
		self.iConst10 = i32::wrapping_add((0.035306122 * self.fConst0) as i32, 23);
		self.iConst11 = i32::wrapping_add((0.033809524 * self.fConst0) as i32, 23);
		self.iConst12 = i32::wrapping_add((0.0322449 * self.fConst0) as i32, 23);
		self.iConst13 = i32::wrapping_add((0.030748298 * self.fConst0) as i32, 23);
		self.iConst14 = i32::wrapping_add((0.028956916 * self.fConst0) as i32, 23);
		self.iConst15 = i32::wrapping_add((0.026938776 * self.fConst0) as i32, 23);
		self.iConst16 = i32::wrapping_add((0.025306122 * self.fConst0) as i32, 23);
		self.iConst17 = std::cmp::min(1024, std::cmp::max(0, i32::wrapping_add((0.0126077095 * self.fConst0) as i32, 22)));
		self.iConst18 = std::cmp::min(1024, std::cmp::max(0, i32::wrapping_add((0.01 * self.fConst0) as i32, 22)));
		self.iConst19 = std::cmp::min(1024, std::cmp::max(0, i32::wrapping_add((0.0077324263 * self.fConst0) as i32, 22)));
		self.iConst20 = std::cmp::min(1024, std::cmp::max(0, i32::wrapping_add((0.0051020407 * self.fConst0) as i32, 22)));
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		mydsp::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("gest_synth");
		ui_interface.add_horizontal_slider("cutoff", ParamIndex(0), 2e+03, 3e+01, 1e+04, 0.01);
		ui_interface.add_horizontal_slider("detune", ParamIndex(1), 1.001, 0.99, 1.05, 0.001);
		ui_interface.add_horizontal_slider("filter_type", ParamIndex(2), 0.0, 0.0, 2.0, 1.0);
		ui_interface.add_horizontal_slider("freq", ParamIndex(3), 4.4e+02, 4e+01, 2e+03, 0.01);
		ui_interface.add_horizontal_slider("gain", ParamIndex(4), 0.3, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("mod_amt", ParamIndex(5), 0.05, 0.0, 0.6, 0.01);
		ui_interface.add_horizontal_slider("mod_freq", ParamIndex(6), 3.0, 0.1, 3e+01, 0.01);
		ui_interface.add_horizontal_slider("osc_type", ParamIndex(7), 0.0, 0.0, 3.0, 1.0);
		ui_interface.add_horizontal_slider("pan", ParamIndex(8), 0.5, 0.0, 1.0, 0.01);
		ui_interface.add_horizontal_slider("reverb_mix", ParamIndex(9), 0.15, 0.0, 0.8, 0.01);
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			0 => Some(self.fHslider0),
			7 => Some(self.fHslider1),
			6 => Some(self.fHslider2),
			5 => Some(self.fHslider3),
			3 => Some(self.fHslider4),
			1 => Some(self.fHslider5),
			2 => Some(self.fHslider6),
			9 => Some(self.fHslider7),
			4 => Some(self.fHslider8),
			8 => Some(self.fHslider9),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			0 => { self.fHslider0 = value }
			7 => { self.fHslider1 = value }
			6 => { self.fHslider2 = value }
			5 => { self.fHslider3 = value }
			3 => { self.fHslider4 = value }
			1 => { self.fHslider5 = value }
			2 => { self.fHslider6 = value }
			9 => { self.fHslider7 = value }
			4 => { self.fHslider8 = value }
			8 => { self.fHslider9 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = self.fHslider0;
		let mut fSlow1: F32 = F32::tan(self.fConst1 * fSlow0);
		let mut fSlow2: F32 = mydsp_faustpower2_f(fSlow1);
		let mut fSlow3: F32 = 2.0 * (1.0 - 1.0 / fSlow2);
		let mut fSlow4: F32 = 1.0 / fSlow1;
		let mut fSlow5: F32 = (fSlow4 + -1.4142135) / fSlow1 + 1.0;
		let mut fSlow6: F32 = (fSlow4 + 1.4142135) / fSlow1 + 1.0;
		let mut fSlow7: F32 = 1.0 / fSlow6;
		let mut fSlow8: F32 = self.fHslider1;
		let mut iSlow9: i32 = (fSlow8 > 0.0) as i32;
		let mut iSlow10: i32 = (fSlow8 > 1.0) as i32;
		let mut iSlow11: i32 = (fSlow8 > 2.0) as i32;
		let mut fSlow12: F32 = self.fConst2 * self.fHslider2;
		let mut fSlow13: F32 = 0.5 * self.fHslider3;
		let mut fSlow14: F32 = self.fHslider4;
		let mut fSlow15: F32 = self.fConst5 * fSlow14;
		let mut fSlow16: F32 = self.fConst2 * fSlow14;
		let mut fSlow17: F32 = 0.5 * fSlow14;
		let mut fSlow18: F32 = F32::max(fSlow17, 23.44895);
		let mut fSlow19: F32 = F32::max(2e+01, F32::abs(fSlow18));
		let mut fSlow20: F32 = self.fConst2 * fSlow19;
		let mut fSlow21: F32 = self.fConst4 / fSlow19;
		let mut fSlow22: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fSlow18));
		let mut iSlow23: i32 = (fSlow22) as i32;
		let mut iSlow24: i32 = i32::wrapping_add(iSlow23, 1);
		let mut fSlow25: F32 = F32::floor(fSlow22);
		let mut fSlow26: F32 = fSlow22 - fSlow25;
		let mut fSlow27: F32 = fSlow25 + (1.0 - fSlow22);
		let mut fSlow28: F32 = self.fConst6 * fSlow14;
		let mut fSlow29: F32 = F32::max(1.1920929e-07, F32::abs(fSlow17));
		let mut fSlow30: F32 = self.fConst2 * fSlow29;
		let mut fSlow31: F32 = 1.0 - self.fConst0 / fSlow29;
		let mut fSlow32: F32 = self.fConst7 * fSlow14;
		let mut fSlow33: F32 = self.fHslider5 * fSlow14;
		let mut fSlow34: F32 = F32::max(fSlow33, 23.44895);
		let mut fSlow35: F32 = F32::max(2e+01, F32::abs(fSlow34));
		let mut fSlow36: F32 = self.fConst2 * fSlow35;
		let mut fSlow37: F32 = self.fConst4 / fSlow35;
		let mut fSlow38: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fSlow34));
		let mut iSlow39: i32 = (fSlow38) as i32;
		let mut iSlow40: i32 = i32::wrapping_add(iSlow39, 1);
		let mut fSlow41: F32 = F32::floor(fSlow38);
		let mut fSlow42: F32 = fSlow38 - fSlow41;
		let mut fSlow43: F32 = fSlow41 + (1.0 - fSlow38);
		let mut fSlow44: F32 = self.fConst5 * fSlow33;
		let mut fSlow45: F32 = F32::max(1.1920929e-07, F32::abs(fSlow33));
		let mut fSlow46: F32 = self.fConst2 * fSlow45;
		let mut fSlow47: F32 = 1.0 - self.fConst0 / fSlow45;
		let mut fSlow48: F32 = self.fConst2 * fSlow33;
		let mut fSlow49: F32 = self.fHslider6;
		let mut iSlow50: i32 = (fSlow49 > 1.0) as i32;
		let mut fSlow51: F32 = (i32::wrapping_sub(iSlow50, (fSlow49 > 2.0) as i32)) as F32;
		let mut fSlow52: F32 = F32::tan(self.fConst8 * fSlow0);
		let mut fSlow53: F32 = 2.0 * (1.0 - 1.0 / mydsp_faustpower2_f(fSlow52));
		let mut fSlow54: F32 = 1.0 / fSlow52;
		let mut fSlow55: F32 = (fSlow54 + -1.4142135) / fSlow52 + 1.0;
		let mut fSlow56: F32 = 1.0 / ((fSlow54 + 1.4142135) / fSlow52 + 1.0);
		let mut iSlow57: i32 = (fSlow49 > 0.0) as i32;
		let mut fSlow58: F32 = (i32::wrapping_sub(iSlow57, iSlow50)) as F32;
		let mut fSlow59: F32 = 1.0 / (fSlow2 * fSlow6);
		let mut fSlow60: F32 = (fSlow4 + -1.0) / fSlow1 + 1.0;
		let mut fSlow61: F32 = (fSlow4 + 1.0) / fSlow1 + 1.0;
		let mut fSlow62: F32 = 1.0 / fSlow61;
		let mut fSlow63: F32 = 1.0 - fSlow4;
		let mut fSlow64: F32 = 1.0 / (fSlow4 + 1.0);
		let mut fSlow65: F32 = (1.0 - (((fSlow49 < 0.0) as i32) as u32 as F32 + (iSlow57) as F32)) / fSlow61;
		let mut fSlow66: F32 = self.fHslider7;
		let mut fSlow67: F32 = 1.0 - fSlow66;
		let mut fSlow68: F32 = self.fHslider8;
		let mut fSlow69: F32 = self.fHslider9;
		let mut fSlow70: F32 = fSlow69 * fSlow68;
		let mut fSlow71: F32 = (1.0 - fSlow69) * fSlow68;
		let zipped_iterators = outputs0.zip(outputs1);
		for (output0, output1) in zipped_iterators {
			self.iVec0[0] = 1;
			self.iRec1[0] = i32::wrapping_add(i32::wrapping_mul(1103515245, self.iRec1[1]), 12345);
			let mut iTemp0: i32 = i32::wrapping_sub(1, self.iVec0[1]);
			let mut fTemp1: F32 = if iTemp0 != 0 {0.0} else {fSlow12 + self.fRec3[1]};
			self.fRec3[0] = fTemp1 - F32::floor(fTemp1);
			let mut fTemp2: F32 = fSlow13 * unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec3[0]) as i32, 65535))) as usize] } + 1.0;
			let mut fTemp3: F32 = fSlow14 * fTemp2;
			let mut fTemp4: F32 = F32::max(fTemp3, 23.44895);
			let mut fTemp5: F32 = F32::max(2e+01, F32::abs(fTemp4));
			let mut fTemp6: F32 = if iTemp0 != 0 {0.0} else {self.fRec5[1] + self.fConst2 * fTemp5};
			self.fRec5[0] = fTemp6 - F32::floor(fTemp6);
			let mut fTemp7: F32 = mydsp_faustpower2_f(2.0 * self.fRec5[0] + -1.0);
			self.fVec2[0] = fTemp7;
			let mut fTemp8: F32 = (self.iVec0[1]) as F32;
			let mut fTemp9: F32 = fTemp8 * (fTemp7 - self.fVec2[1]) / fTemp5;
			self.fVec3[(self.IOTA0 & 4095) as usize] = fTemp9;
			let mut fTemp10: F32 = F32::max(0.0, F32::min(2047.0, self.fConst3 / fTemp4));
			let mut iTemp11: i32 = (fTemp10) as i32;
			let mut fTemp12: F32 = F32::floor(fTemp10);
			let mut fTemp13: F32 = self.fConst4 * (fTemp9 - self.fVec3[((i32::wrapping_sub(self.IOTA0, iTemp11)) & 4095) as usize] * (fTemp12 + (1.0 - fTemp10)) - (fTemp10 - fTemp12) * self.fVec3[((i32::wrapping_sub(self.IOTA0, i32::wrapping_add(iTemp11, 1))) & 4095) as usize]);
			self.fRec4[0] = 0.999 * self.fRec4[1] + fTemp13;
			let mut fTemp14: F32 = F32::max(1.1920929e-07, F32::abs(fTemp3));
			let mut fTemp15: F32 = self.fRec6[1] + self.fConst2 * fTemp14;
			let mut fTemp16: F32 = fTemp15 + -1.0;
			let mut iTemp17: i32 = (fTemp16 < 0.0) as i32;
			self.fRec6[0] = if iTemp17 != 0 {fTemp15} else {fTemp16};
			let mut fRec7: F32 = if iTemp17 != 0 {fTemp15} else {fTemp15 + fTemp16 * (1.0 - self.fConst0 / fTemp14)};
			let mut fTemp18: F32 = if iTemp0 != 0 {0.0} else {fSlow16 * fTemp2 + self.fRec8[1]};
			self.fRec8[0] = fTemp18 - F32::floor(fTemp18);
			let mut fTemp19: F32 = if iTemp0 != 0 {0.0} else {fSlow20 + self.fRec10[1]};
			self.fRec10[0] = fTemp19 - F32::floor(fTemp19);
			let mut fTemp20: F32 = mydsp_faustpower2_f(2.0 * self.fRec10[0] + -1.0);
			self.fVec4[0] = fTemp20;
			let mut fTemp21: F32 = fSlow21 * fTemp8 * (fTemp20 - self.fVec4[1]);
			self.fVec5[(self.IOTA0 & 4095) as usize] = fTemp21;
			let mut fTemp22: F32 = fSlow27 * self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow23)) & 4095) as usize] + fSlow26 * self.fVec5[((i32::wrapping_sub(self.IOTA0, iSlow24)) & 4095) as usize];
			self.fRec9[0] = 0.999 * self.fRec9[1] + fTemp21 - fTemp22;
			let mut fTemp23: F32 = fSlow30 + self.fRec11[1] + -1.0;
			let mut iTemp24: i32 = (fTemp23 < 0.0) as i32;
			let mut fTemp25: F32 = fSlow30 + self.fRec11[1];
			self.fRec11[0] = if iTemp24 != 0 {fTemp25} else {fTemp23};
			let mut fRec12: F32 = if iTemp24 != 0 {fTemp25} else {fSlow30 + self.fRec11[1] + fSlow31 * fTemp23};
			let mut fTemp26: F32 = if iTemp0 != 0 {0.0} else {fSlow32 + self.fRec13[1]};
			self.fRec13[0] = fTemp26 - F32::floor(fTemp26);
			let mut fTemp27: F32 = if iTemp0 != 0 {0.0} else {fSlow36 + self.fRec15[1]};
			self.fRec15[0] = fTemp27 - F32::floor(fTemp27);
			let mut fTemp28: F32 = mydsp_faustpower2_f(2.0 * self.fRec15[0] + -1.0);
			self.fVec6[0] = fTemp28;
			let mut fTemp29: F32 = fSlow37 * fTemp8 * (fTemp28 - self.fVec6[1]);
			self.fVec7[(self.IOTA0 & 4095) as usize] = fTemp29;
			let mut fTemp30: F32 = fSlow43 * self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow39)) & 4095) as usize] + fSlow42 * self.fVec7[((i32::wrapping_sub(self.IOTA0, iSlow40)) & 4095) as usize];
			self.fRec14[0] = 0.999 * self.fRec14[1] + fTemp29 - fTemp30;
			let mut fTemp31: F32 = fSlow46 + self.fRec16[1] + -1.0;
			let mut iTemp32: i32 = (fTemp31 < 0.0) as i32;
			let mut fTemp33: F32 = fSlow46 + self.fRec16[1];
			self.fRec16[0] = if iTemp32 != 0 {fTemp33} else {fTemp31};
			let mut fRec17: F32 = if iTemp32 != 0 {fTemp33} else {fSlow46 + self.fRec16[1] + fSlow47 * fTemp31};
			let mut fTemp34: F32 = if iTemp0 != 0 {0.0} else {fSlow48 + self.fRec18[1]};
			self.fRec18[0] = fTemp34 - F32::floor(fTemp34);
			let mut fTemp35: F32 = 0.3 * if iSlow9 != 0 {unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec18[0]) as i32, 65535))) as usize] }} else {if iSlow10 != 0 {2.0 * fRec17 + -1.0} else {if iSlow11 != 0 {fTemp29 - fTemp30} else {fSlow44 * self.fRec14[0]}}} + 0.25 * if iSlow9 != 0 {unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec13[0]) as i32, 65535))) as usize] }} else {if iSlow10 != 0 {2.0 * fRec12 + -1.0} else {if iSlow11 != 0 {fTemp21 - fTemp22} else {fSlow28 * self.fRec9[0]}}} + if iSlow9 != 0 {unsafe { ftbl0mydspSIG0[(std::cmp::max(0, std::cmp::min((65536.0 * self.fRec8[0]) as i32, 65535))) as usize] }} else {if iSlow10 != 0 {2.0 * fRec7 + -1.0} else {if iSlow11 != 0 {fTemp13} else {fSlow15 * self.fRec4[0] * fTemp2}}} + 1.8626451e-11 * (self.iRec1[0]) as F32;
			self.fVec8[0] = fTemp35;
			self.fRec0[0] = fTemp35 - fSlow7 * (fSlow5 * self.fRec0[2] + fSlow3 * self.fRec0[1]);
			let mut fTemp36: F32 = 2.0 * self.fRec0[1];
			self.fRec20[0] = fTemp35 - fSlow56 * (fSlow55 * self.fRec20[2] + fSlow53 * self.fRec20[1]);
			self.fRec19[0] = fSlow56 * (self.fRec20[2] + self.fRec20[0] + 2.0 * self.fRec20[1]) - fSlow7 * (fSlow5 * self.fRec19[2] + fSlow3 * self.fRec19[1]);
			let mut fTemp37: F32 = 2.0 * self.fRec19[1];
			self.fRec22[0] = -(fSlow64 * (fSlow63 * self.fRec22[1] - (fTemp35 + self.fVec8[1])));
			self.fRec21[0] = self.fRec22[0] - fSlow62 * (fSlow60 * self.fRec21[2] + fSlow3 * self.fRec21[1]);
			let mut fTemp38: F32 = fSlow65 * (self.fRec21[2] + self.fRec21[0] + 2.0 * self.fRec21[1]);
			self.fRec32[0] = 0.3 * self.fRec32[1] + 0.7 * self.fRec31[1];
			let mut fTemp39: F32 = fSlow59 * (fSlow58 * (self.fRec19[2] + (self.fRec19[0] - fTemp37)) + fSlow51 * (self.fRec0[2] + (self.fRec0[0] - fTemp36)));
			self.fVec9[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec32[0];
			self.fRec31[0] = self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst9)) & 8191) as usize];
			self.fRec34[0] = 0.3 * self.fRec34[1] + 0.7 * self.fRec33[1];
			self.fVec10[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec34[0];
			self.fRec33[0] = self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst10)) & 8191) as usize];
			self.fRec36[0] = 0.3 * self.fRec36[1] + 0.7 * self.fRec35[1];
			self.fVec11[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec36[0];
			self.fRec35[0] = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst11)) & 8191) as usize];
			self.fRec38[0] = 0.3 * self.fRec38[1] + 0.7 * self.fRec37[1];
			self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec38[0];
			self.fRec37[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst12)) & 8191) as usize];
			self.fRec40[0] = 0.3 * self.fRec40[1] + 0.7 * self.fRec39[1];
			self.fVec13[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec40[0];
			self.fRec39[0] = self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst13)) & 8191) as usize];
			self.fRec42[0] = 0.3 * self.fRec42[1] + 0.7 * self.fRec41[1];
			self.fVec14[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec42[0];
			self.fRec41[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst14)) & 8191) as usize];
			self.fRec44[0] = 0.3 * self.fRec44[1] + 0.7 * self.fRec43[1];
			self.fVec15[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec44[0];
			self.fRec43[0] = self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst15)) & 8191) as usize];
			self.fRec46[0] = 0.3 * self.fRec46[1] + 0.7 * self.fRec45[1];
			self.fVec16[(self.IOTA0 & 8191) as usize] = fTemp39 + fTemp38 + 0.5 * self.fRec46[0];
			self.fRec45[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst16)) & 8191) as usize];
			let mut fTemp40: F32 = self.fRec45[0] + self.fRec43[0] + self.fRec41[0] + self.fRec39[0] + self.fRec37[0] + self.fRec35[0] + self.fRec33[0] + self.fRec31[0] + 0.3 * self.fRec29[1];
			self.fVec17[(self.IOTA0 & 2047) as usize] = fTemp40;
			self.fRec29[0] = self.fVec17[((i32::wrapping_sub(self.IOTA0, self.iConst17)) & 2047) as usize];
			let mut fRec30: F32 = -(0.3 * fTemp40);
			let mut fTemp41: F32 = self.fRec29[1] + fRec30 + 0.3 * self.fRec27[1];
			self.fVec18[(self.IOTA0 & 2047) as usize] = fTemp41;
			self.fRec27[0] = self.fVec18[((i32::wrapping_sub(self.IOTA0, self.iConst18)) & 2047) as usize];
			let mut fRec28: F32 = -(0.3 * fTemp41);
			let mut fTemp42: F32 = self.fRec27[1] + fRec28 + 0.3 * self.fRec25[1];
			self.fVec19[(self.IOTA0 & 2047) as usize] = fTemp42;
			self.fRec25[0] = self.fVec19[((i32::wrapping_sub(self.IOTA0, self.iConst19)) & 2047) as usize];
			let mut fRec26: F32 = -(0.3 * fTemp42);
			let mut fTemp43: F32 = self.fRec25[1] + fRec26 + 0.3 * self.fRec23[1];
			self.fVec20[(self.IOTA0 & 1023) as usize] = fTemp43;
			self.fRec23[0] = self.fVec20[((i32::wrapping_sub(self.IOTA0, self.iConst20)) & 1023) as usize];
			let mut fRec24: F32 = -(0.3 * fTemp43);
			let mut fTemp44: F32 = fSlow66 * (fRec24 + self.fRec23[1]) + fSlow67 * (fTemp38 + fSlow59 * (fSlow58 * (self.fRec19[0] + self.fRec19[2] - fTemp37) + fSlow51 * (self.fRec0[0] + self.fRec0[2] - fTemp36)));
			*output0 = fSlow70 * fTemp44;
			*output1 = fSlow71 * fTemp44;
			self.iVec0[1] = self.iVec0[0];
			self.iRec1[1] = self.iRec1[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec5[1] = self.fRec5[0];
			self.fVec2[1] = self.fVec2[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec4[1] = self.fRec4[0];
			self.fRec6[1] = self.fRec6[0];
			self.fRec8[1] = self.fRec8[0];
			self.fRec10[1] = self.fRec10[0];
			self.fVec4[1] = self.fVec4[0];
			self.fRec9[1] = self.fRec9[0];
			self.fRec11[1] = self.fRec11[0];
			self.fRec13[1] = self.fRec13[0];
			self.fRec15[1] = self.fRec15[0];
			self.fVec6[1] = self.fVec6[0];
			self.fRec14[1] = self.fRec14[0];
			self.fRec16[1] = self.fRec16[0];
			self.fRec18[1] = self.fRec18[0];
			self.fVec8[1] = self.fVec8[0];
			self.fRec0[2] = self.fRec0[1];
			self.fRec0[1] = self.fRec0[0];
			self.fRec20[2] = self.fRec20[1];
			self.fRec20[1] = self.fRec20[0];
			self.fRec19[2] = self.fRec19[1];
			self.fRec19[1] = self.fRec19[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec21[2] = self.fRec21[1];
			self.fRec21[1] = self.fRec21[0];
			self.fRec32[1] = self.fRec32[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec36[1] = self.fRec36[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec40[1] = self.fRec40[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec44[1] = self.fRec44[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
		}
	}

}

